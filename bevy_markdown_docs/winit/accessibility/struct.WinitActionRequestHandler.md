[bevy](../../index.html)::[winit](../index.html)::[accessibility](index.html)

# Struct WinitActionRequestHandler 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#46)

```rust
pub struct WinitActionRequestHandler(pub VecDeque<ActionRequest>);
```

Forwards `AccessKit` [`ActionRequest`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.ActionRequest.html "struct accesskit::ActionRequest")s from winit to an event channel.

## Tuple Fields

`0: [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<[ActionRequest](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.ActionRequest.html "struct accesskit::ActionRequest")>`

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<[ActionRequest](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.ActionRequest.html "struct accesskit::ActionRequest")\>>

[Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#803-806)

#### pub fn [extract\_if](#method.extract_if)<F, R>( &mut self, range: R, filter: F, ) -> [ExtractIf](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/extract_if/struct.ExtractIf.html "struct alloc::collections::vec_deque::extract_if::ExtractIf")<'\_, T, F, A> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

🔬This is a nightly-only experimental API. (`vec_deque_extract_if`)

Creates an iterator which uses a closure to determine if an element in the range should be removed.

If the closure returns `true`, the element is removed from the deque and yielded. If the closure returns `false`, or panics, the element remains in the deque and will not be yielded.

Only elements that fall in the provided range are considered for extraction, but any elements after the range will still have to be moved if any element has been extracted.

If the returned `ExtractIf` is not exhausted, e.g. because it is dropped without iterating or the iteration short-circuits, then the remaining elements will be retained. Use `extract_if().for_each(drop)` if you do not need the returned iterator, or [`retain_mut`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.retain_mut "method alloc::collections::vec_deque::VecDeque::retain_mut") with a negated predicate if you also do not need to restrict the range.

Using this method is equivalent to the following code:

```rust
#![feature(vec_deque_extract_if)]
let mut i = range.start;
let end_items = deq.len() - range.end;

while i < deq.len() - end_items {
    if some_predicate(&mut deq[i]) {
        let val = deq.remove(i).unwrap();
        // your code here
    } else {
        i += 1;
    }
}
```

But `extract_if` is easier to use. `extract_if` is also more efficient, because it can backshift the elements of the array in bulk.

The iterator also lets you mutate the value of each element in the closure, regardless of whether you choose to keep or remove it.

##### Panics

If `range` is out of bounds.

##### Examples

Splitting a deque into even and odd values, reusing the original deque:

```rust
#![feature(vec_deque_extract_if)]
use std::collections::VecDeque;

let mut numbers = VecDeque::from([1, 2, 3, 4, 5, 6, 8, 9, 11, 13, 14, 15]);

let evens = numbers.extract_if(.., |x| *x % 2 == 0).collect::<VecDeque<_>>();
let odds = numbers;

assert_eq!(evens, VecDeque::from([2, 4, 6, 8, 14]));
assert_eq!(odds, VecDeque::from([1, 3, 5, 9, 11, 13, 15]));
```

Using the range argument to only process a part of the deque:

```rust
#![feature(vec_deque_extract_if)]
use std::collections::VecDeque;

let mut items = VecDeque::from([0, 0, 0, 0, 0, 0, 0, 1, 2, 1, 2, 1, 2]);
let ones = items.extract_if(7.., |x| *x == 1).collect::<VecDeque<_>>();
assert_eq!(items, VecDeque::from([0, 0, 0, 0, 0, 0, 0, 2, 2, 2]));
assert_eq!(ones.len(), 3);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#966)

#### pub fn [get](#method.get)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Provides a reference to the element at the given index.

Element at index 0 is the front of the queue.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.push_back(3);
buf.push_back(4);
buf.push_back(5);
buf.push_back(6);
assert_eq!(buf.get(1), Some(&4));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#996)

#### pub fn [get\_mut](#method.get_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Provides a mutable reference to the element at the given index.

Element at index 0 is the front of the queue.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.push_back(3);
buf.push_back(4);
buf.push_back(5);
buf.push_back(6);
assert_eq!(buf[1], 4);
if let Some(elem) = buf.get_mut(1) {
    *elem = 7;
}
assert_eq!(buf[1], 7);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1029)

#### pub fn [swap](#method.swap)(&mut self, i: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), j: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Swaps elements at indices `i` and `j`.

`i` and `j` may be equal.

Element at index 0 is the front of the queue.

##### Panics

Panics if either index is out of bounds.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.push_back(3);
buf.push_back(4);
buf.push_back(5);
assert_eq!(buf, [3, 4, 5]);
buf.swap(0, 2);
assert_eq!(buf, [5, 4, 3]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1050)

#### pub fn [capacity](#method.capacity)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements the deque can hold without reallocating.

##### Examples

```rust
use std::collections::VecDeque;

let buf: VecDeque<i32> = VecDeque::with_capacity(10);
assert!(buf.capacity() >= 10);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1077)

#### pub fn [reserve\_exact](#method.reserve_exact)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Reserves the minimum capacity for at least `additional` more elements to be inserted in the given deque. Does nothing if the capacity is already sufficient.

Note that the allocator may give the collection more space than it requests. Therefore capacity can not be relied upon to be precisely minimal. Prefer [`reserve`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.reserve "method alloc::collections::vec_deque::VecDeque::reserve") if future insertions are expected.

##### Panics

Panics if the new capacity overflows `usize`.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf: VecDeque<i32> = [1].into();
buf.reserve_exact(10);
assert!(buf.capacity() >= 11);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1107)

#### pub fn [reserve](#method.reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Reserves capacity for at least `additional` more elements to be inserted in the given deque. The collection may reserve more space to speculatively avoid frequent reallocations.

##### Panics

Panics if the new capacity overflows `usize`.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf: VecDeque<i32> = [1].into();
buf.reserve(10);
assert!(buf.capacity() >= 11);
```

1.57.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1159)

#### pub fn [try\_reserve\_exact](#method.try_reserve_exact)( &mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryReserveError](https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html "struct alloc::collections::TryReserveError")\>

Tries to reserve the minimum capacity for at least `additional` more elements to be inserted in the given deque. After calling `try_reserve_exact`, capacity will be greater than or equal to `self.len() + additional` if it returns `Ok(())`. Does nothing if the capacity is already sufficient.

Note that the allocator may give the collection more space than it requests. Therefore, capacity can not be relied upon to be precisely minimal. Prefer [`try_reserve`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.try_reserve "method alloc::collections::vec_deque::VecDeque::try_reserve") if future insertions are expected.

##### Errors

If the capacity overflows `usize`, or the allocator reports a failure, then an error is returned.

##### Examples

```rust
use std::collections::TryReserveError;
use std::collections::VecDeque;

fn process_data(data: &[u32]) -> Result<VecDeque<u32>, TryReserveError> {
    let mut output = VecDeque::new();

    // Pre-reserve the memory, exiting if we can't
    output.try_reserve_exact(data.len())?;

    // Now we know this can't OOM(Out-Of-Memory) in the middle of our complex work
    output.extend(data.iter().map(|&val| {
        val * 2 + 5 // very complicated
    }));

    Ok(output)
}
```

1.57.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1207)

#### pub fn [try\_reserve](#method.try_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryReserveError](https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html "struct alloc::collections::TryReserveError")\>

Tries to reserve capacity for at least `additional` more elements to be inserted in the given deque. The collection may reserve more space to speculatively avoid frequent reallocations. After calling `try_reserve`, capacity will be greater than or equal to `self.len() + additional` if it returns `Ok(())`. Does nothing if capacity is already sufficient. This method preserves the contents even if an error occurs.

##### Errors

If the capacity overflows `usize`, or the allocator reports a failure, then an error is returned.

##### Examples

```rust
use std::collections::TryReserveError;
use std::collections::VecDeque;

fn process_data(data: &[u32]) -> Result<VecDeque<u32>, TryReserveError> {
    let mut output = VecDeque::new();

    // Pre-reserve the memory, exiting if we can't
    output.try_reserve(data.len())?;

    // Now we know this can't OOM in the middle of our complex work
    output.extend(data.iter().map(|&val| {
        val * 2 + 5 // very complicated
    }));

    Ok(output)
}
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1238)

#### pub fn [shrink\_to\_fit](#method.shrink_to_fit)(&mut self)

Shrinks the capacity of the deque as much as possible.

It will drop down as close as possible to the length but the allocator may still inform the deque that there is space for a few more elements.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::with_capacity(15);
buf.extend(0..4);
assert_eq!(buf.capacity(), 15);
buf.shrink_to_fit();
assert!(buf.capacity() >= 4);
```

1.56.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1263)

#### pub fn [shrink\_to](#method.shrink_to)(&mut self, min\_capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Shrinks the capacity of the deque with a lower bound.

The capacity will remain at least as large as both the length and the supplied value.

If the current capacity is less than the lower limit, this is a no-op.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::with_capacity(15);
buf.extend(0..4);
assert_eq!(buf.capacity(), 15);
buf.shrink_to(6);
assert!(buf.capacity() >= 6);
buf.shrink_to(0);
assert!(buf.capacity() >= 4);
```

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1434)

#### pub fn [truncate](#method.truncate)(&mut self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Shortens the deque, keeping the first `len` elements and dropping the rest.

If `len` is greater or equal to the deque’s current length, this has no effect.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.push_back(5);
buf.push_back(10);
buf.push_back(15);
assert_eq!(buf, [5, 10, 15]);
buf.truncate(1);
assert_eq!(buf, [5]);
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1500)

#### pub fn [truncate\_front](#method.truncate_front)(&mut self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`vec_deque_truncate_front`)

Shortens the deque, keeping the last `len` elements and dropping the rest.

If `len` is greater or equal to the deque’s current length, this has no effect.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.push_front(5);
buf.push_front(10);
buf.push_front(15);
assert_eq!(buf, [15, 10, 5]);
assert_eq!(buf.as_slices(), (&[15, 10, 5][..], &[][..]));
buf.truncate_front(1);
assert_eq!(buf.as_slices(), (&[5][..], &[][..]));
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1548)

#### pub fn [allocator](#method.allocator)(&self) -> [&A](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

🔬This is a nightly-only experimental API. (`allocator_api`)

Returns a reference to the underlying allocator.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1569)

#### pub fn [iter](#method.iter)(&self) -> [Iter](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/iter/struct.Iter.html "struct alloc::collections::vec_deque::iter::Iter")<'\_, T> [ⓘ](#)

Returns a front-to-back iterator.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.push_back(5);
buf.push_back(3);
buf.push_back(4);
let b: &[_] = &[&5, &3, &4];
let c: Vec<&i32> = buf.iter().collect();
assert_eq!(&c[..], b);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1592)

#### pub fn [iter\_mut](#method.iter_mut)(&mut self) -> [IterMut](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/iter_mut/struct.IterMut.html "struct alloc::collections::vec_deque::iter_mut::IterMut")<'\_, T> [ⓘ](#)

Returns a front-to-back iterator that returns mutable references.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.push_back(5);
buf.push_back(3);
buf.push_back(4);
for num in buf.iter_mut() {
    *num = *num - 2;
}
let b: &[_] = &[&mut 3, &mut 1, &mut 2];
assert_eq!(&buf.iter_mut().collect::<Vec<&mut i32>>()[..], b);
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1633)

#### pub fn [as\_slices](#method.as_slices)(&self) -> (&[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &[\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

Returns a pair of slices which contain, in order, the contents of the deque.

If [`make_contiguous`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.make_contiguous "method alloc::collections::vec_deque::VecDeque::make_contiguous") was previously called, all elements of the deque will be in the first slice and the second slice will be empty. Otherwise, the exact split point depends on implementation details and is not guaranteed.

##### Examples

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();

deque.push_back(0);
deque.push_back(1);
deque.push_back(2);

let expected = [0, 1, 2];
let (front, back) = deque.as_slices();
assert_eq!(&expected[..front.len()], front);
assert_eq!(&expected[front.len()..], back);

deque.push_front(10);
deque.push_front(9);

let expected = [9, 10, 0, 1, 2];
let (front, back) = deque.as_slices();
assert_eq!(&expected[..front.len()], front);
assert_eq!(&expected[front.len()..], back);
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1682)

#### pub fn [as\_mut\_slices](#method.as_mut_slices)(&mut self) -> (&mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html), &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

Returns a pair of slices which contain, in order, the contents of the deque.

If [`make_contiguous`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.make_contiguous "method alloc::collections::vec_deque::VecDeque::make_contiguous") was previously called, all elements of the deque will be in the first slice and the second slice will be empty. Otherwise, the exact split point depends on implementation details and is not guaranteed.

##### Examples

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();

deque.push_back(0);
deque.push_back(1);

deque.push_front(10);
deque.push_front(9);

// Since the split point is not guaranteed, we may need to update
// either slice.
let mut update_nth = |index: usize, val: u32| {
    let (front, back) = deque.as_mut_slices();
    if index > front.len() - 1 {
        back[index - front.len()] = val;
    } else {
        front[index] = val;
    }
};

update_nth(0, 42);
update_nth(2, 24);

let v: Vec<_> = deque.into();
assert_eq!(v, [42, 10, 24, 1]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1703)

#### pub fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements in the deque.

##### Examples

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();
assert_eq!(deque.len(), 0);
deque.push_back(1);
assert_eq!(deque.len(), 1);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1720)

#### pub fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the deque is empty.

##### Examples

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();
assert!(deque.is_empty());
deque.push_front(1);
assert!(!deque.is_empty());
```

1.51.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1789-1791)

#### pub fn [range](#method.range)<R>(&self, range: R) -> [Iter](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/iter/struct.Iter.html "struct alloc::collections::vec_deque::iter::Iter")<'\_, T> [ⓘ](#)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Creates an iterator that covers the specified range in the deque.

##### Panics

Panics if the range has `start_bound > end_bound`, or, if the range is bounded on either end and past the length of the deque.

##### Examples

```rust
use std::collections::VecDeque;

let deque: VecDeque<_> = [1, 2, 3].into();
let range = deque.range(2..).copied().collect::<VecDeque<_>>();
assert_eq!(range, [3]);

// A full range covers all contents
let all = deque.range(..);
assert_eq!(all.len(), 3);
```

1.51.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1829-1831)

#### pub fn [range\_mut](#method.range_mut)<R>(&mut self, range: R) -> [IterMut](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/iter_mut/struct.IterMut.html "struct alloc::collections::vec_deque::iter_mut::IterMut")<'\_, T> [ⓘ](#)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Creates an iterator that covers the specified mutable range in the deque.

##### Panics

Panics if the range has `start_bound > end_bound`, or, if the range is bounded on either end and past the length of the deque.

##### Examples

```rust
use std::collections::VecDeque;

let mut deque: VecDeque<_> = [1, 2, 3].into();
for v in deque.range_mut(2..) {
  *v *= 2;
}
assert_eq!(deque, [1, 2, 6]);

// A full range covers all contents
for v in deque.range_mut(..) {
  *v *= 2;
}
assert_eq!(deque, [2, 4, 12]);
```

1.6.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1878-1880)

#### pub fn [drain](#method.drain)<R>(&mut self, range: R) -> [Drain](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/drain/struct.Drain.html "struct alloc::collections::vec_deque::drain::Drain")<'\_, T, A> [ⓘ](#)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Removes the specified range from the deque in bulk, returning all removed elements as an iterator. If the iterator is dropped before being fully consumed, it drops the remaining removed elements.

The returned iterator keeps a mutable borrow on the queue to optimize its implementation.

##### Panics

Panics if the range has `start_bound > end_bound`, or, if the range is bounded on either end and past the length of the deque.

##### Leaking

If the returned iterator goes out of scope without being dropped (due to [`mem::forget`](https://doc.rust-lang.org/nightly/core/mem/fn.forget.html "fn core::mem::forget"), for example), the deque may have lost and leaked elements arbitrarily, including elements outside the range.

##### Examples

```rust
use std::collections::VecDeque;

let mut deque: VecDeque<_> = [1, 2, 3].into();
let drained = deque.drain(2..).collect::<VecDeque<_>>();
assert_eq!(drained, [3]);
assert_eq!(deque, [1, 2]);

// A full range clears all contents, like `clear()` does
deque.drain(..);
assert!(deque.is_empty());
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1968-1971)

#### pub fn [splice](#method.splice)<R, I>( &mut self, range: R, replace\_with: I, ) -> [Splice](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/splice/struct.Splice.html "struct alloc::collections::vec_deque::splice::Splice")<'\_, <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"), A> [ⓘ](#)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = T>,

🔬This is a nightly-only experimental API. (`deque_extend_front`)

Creates a splicing iterator that replaces the specified range in the deque with the given `replace_with` iterator and yields the removed items. `replace_with` does not need to be the same length as `range`.

`range` is removed even if the `Splice` iterator is not consumed before it is dropped.

It is unspecified how many elements are removed from the deque if the `Splice` value is leaked.

The input iterator `replace_with` is only consumed when the `Splice` value is dropped.

This is optimal if:

*   The tail (elements in the deque after `range`) is empty,
*   or `replace_with` yields fewer or equal elements than `range`’s length
*   or the lower bound of its `size_hint()` is exact.

Otherwise, a temporary vector is allocated and the tail is moved twice.

##### Panics

Panics if the range has `start_bound > end_bound`, or, if the range is bounded on either end and past the length of the deque.

##### Examples

```rust
let mut v = VecDeque::from(vec![1, 2, 3, 4]);
let new = [7, 8, 9];
let u: Vec<_> = v.splice(1..3, new).collect();
assert_eq!(v, [1, 7, 8, 9, 4]);
assert_eq!(u, [2, 3]);
```

Using `splice` to insert new items into a vector efficiently at a specific position indicated by an empty range:

```rust
let mut v = VecDeque::from(vec![1, 5]);
let new = [2, 3, 4];
v.splice(1..1, new);
assert_eq!(v, [1, 2, 3, 4, 5]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#1990)

#### pub fn [clear](#method.clear)(&mut self)

Clears the deque, removing all values.

##### Examples

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();
deque.push_back(1);
deque.clear();
assert!(deque.is_empty());
```

1.12.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2019-2021)

#### pub fn [contains](#method.contains)(&self, x: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Returns `true` if the deque contains an element equal to the given value.

This operation is _O_(_n_).

Note that if you have a sorted `VecDeque`, [`binary_search`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.binary_search "method alloc::collections::vec_deque::VecDeque::binary_search") may be faster.

##### Examples

```rust
use std::collections::VecDeque;

let mut deque: VecDeque<u32> = VecDeque::new();

deque.push_back(0);
deque.push_back(1);

assert_eq!(deque.contains(&1), true);
assert_eq!(deque.contains(&10), false);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2044)

#### pub fn [front](#method.front)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Provides a reference to the front element, or `None` if the deque is empty.

##### Examples

```rust
use std::collections::VecDeque;

let mut d = VecDeque::new();
assert_eq!(d.front(), None);

d.push_back(1);
d.push_back(2);
assert_eq!(d.front(), Some(&1));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2068)

#### pub fn [front\_mut](#method.front_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Provides a mutable reference to the front element, or `None` if the deque is empty.

##### Examples

```rust
use std::collections::VecDeque;

let mut d = VecDeque::new();
assert_eq!(d.front_mut(), None);

d.push_back(1);
d.push_back(2);
match d.front_mut() {
    Some(x) => *x = 9,
    None => (),
}
assert_eq!(d.front(), Some(&9));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2089)

#### pub fn [back](#method.back)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Provides a reference to the back element, or `None` if the deque is empty.

##### Examples

```rust
use std::collections::VecDeque;

let mut d = VecDeque::new();
assert_eq!(d.back(), None);

d.push_back(1);
d.push_back(2);
assert_eq!(d.back(), Some(&2));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2113)

#### pub fn [back\_mut](#method.back_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Provides a mutable reference to the back element, or `None` if the deque is empty.

##### Examples

```rust
use std::collections::VecDeque;

let mut d = VecDeque::new();
assert_eq!(d.back(), None);

d.push_back(1);
d.push_back(2);
match d.back_mut() {
    Some(x) => *x = 9,
    None => (),
}
assert_eq!(d.back(), Some(&9));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2134)

#### pub fn [pop\_front](#method.pop_front)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Removes the first element and returns it, or `None` if the deque is empty.

##### Examples

```rust
use std::collections::VecDeque;

let mut d = VecDeque::new();
d.push_back(1);
d.push_back(2);

assert_eq!(d.pop_front(), Some(1));
assert_eq!(d.pop_front(), Some(2));
assert_eq!(d.pop_front(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2163)

#### pub fn [pop\_back](#method.pop_back)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Removes the last element from the deque and returns it, or `None` if it is empty.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
assert_eq!(buf.pop_back(), None);
buf.push_back(1);
buf.push_back(3);
assert_eq!(buf.pop_back(), Some(3));
```

1.93.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2192)

#### pub fn [pop\_front\_if](#method.pop_front_if)( &mut self, predicate: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Removes and returns the first element from the deque if the predicate returns `true`, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the predicate returns false or the deque is empty (the predicate will not be called in that case).

##### Examples

```rust
use std::collections::VecDeque;

let mut deque: VecDeque<i32> = vec![0, 1, 2, 3, 4].into();
let pred = |x: &mut i32| *x % 2 == 0;

assert_eq!(deque.pop_front_if(pred), Some(0));
assert_eq!(deque, [1, 2, 3, 4]);
assert_eq!(deque.pop_front_if(pred), None);
```

1.93.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2214)

#### pub fn [pop\_back\_if](#method.pop_back_if)( &mut self, predicate: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Removes and returns the last element from the deque if the predicate returns `true`, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the predicate returns false or the deque is empty (the predicate will not be called in that case).

##### Examples

```rust
use std::collections::VecDeque;

let mut deque: VecDeque<i32> = vec![0, 1, 2, 3, 4].into();
let pred = |x: &mut i32| *x % 2 == 0;

assert_eq!(deque.pop_back_if(pred), Some(4));
assert_eq!(deque, [0, 1, 2, 3]);
assert_eq!(deque.pop_back_if(pred), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2232)

#### pub fn [push\_front](#method.push_front)(&mut self, value: T)

Prepends an element to the deque.

##### Examples

```rust
use std::collections::VecDeque;

let mut d = VecDeque::new();
d.push_front(1);
d.push_front(2);
assert_eq!(d.front(), Some(&2));
```

1.95.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2250)

#### pub fn [push\_front\_mut](#method.push_front_mut)(&mut self, value: T) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Prepends an element to the deque, returning a reference to it.

##### Examples

```rust
use std::collections::VecDeque;

let mut d = VecDeque::from([1, 2, 3]);
let x = d.push_front_mut(8);
*x -= 1;
assert_eq!(d.front(), Some(&7));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2275)

#### pub fn [push\_back](#method.push_back)(&mut self, value: T)

Appends an element to the back of the deque.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.push_back(1);
buf.push_back(3);
assert_eq!(3, *buf.back().unwrap());
```

1.95.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2293)

#### pub fn [push\_back\_mut](#method.push_back_mut)(&mut self, value: T) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Appends an element to the back of the deque, returning a reference to it.

##### Examples

```rust
use std::collections::VecDeque;

let mut d = VecDeque::from([1, 2, 3]);
let x = d.push_back_mut(9);
*x += 1;
assert_eq!(d.back(), Some(&10));
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2334)

#### pub fn [prepend](#method.prepend)<I>(&mut self, other: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = T>, <I as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"): [DoubleEndedIterator](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator"),

🔬This is a nightly-only experimental API. (`deque_extend_front`)

Prepends all contents of the iterator to the front of the deque. The order of the contents is preserved.

To get behavior like [`append`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.append "method alloc::collections::vec_deque::VecDeque::append") where elements are moved from the other collection to this one, use `self.prepend(other.drain(..))`.

##### Examples

```rust
#![feature(deque_extend_front)]
use std::collections::VecDeque;

let mut deque = VecDeque::from([4, 5, 6]);
deque.prepend([1, 2, 3]);
assert_eq!(deque, [1, 2, 3, 4, 5, 6]);
```

Move values between collections like [`append`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.append "method alloc::collections::vec_deque::VecDeque::append") does but prepend to the front:

```rust
#![feature(deque_extend_front)]
use std::collections::VecDeque;

let mut deque1 = VecDeque::from([4, 5, 6]);
let mut deque2 = VecDeque::from([1, 2, 3]);
deque1.prepend(deque2.drain(..));
assert_eq!(deque1, [1, 2, 3, 4, 5, 6]);
assert!(deque2.is_empty());
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2366)

#### pub fn [extend\_front](#method.extend_front)<I>(&mut self, iter: I)

where I: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = T>,

🔬This is a nightly-only experimental API. (`deque_extend_front`)

Prepends all contents of the iterator to the front of the deque, as if [`push_front`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.push_front "method alloc::collections::vec_deque::VecDeque::push_front") was called repeatedly with the values yielded by the iterator.

##### Examples

```rust
#![feature(deque_extend_front)]
use std::collections::VecDeque;

let mut deque = VecDeque::from([4, 5, 6]);
deque.extend_front([3, 2, 1]);
assert_eq!(deque, [1, 2, 3, 4, 5, 6]);
```

This behaves like [`push_front`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.push_front "method alloc::collections::vec_deque::VecDeque::push_front") was called repeatedly:

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::from([4, 5, 6]);
for v in [3, 2, 1] {
    deque.push_front(v);
}
assert_eq!(deque, [1, 2, 3, 4, 5, 6]);
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2401)

#### pub fn [swap\_remove\_front](#method.swap_remove_front)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Removes an element from anywhere in the deque and returns it, replacing it with the first element.

This does not preserve ordering, but is _O_(1).

Returns `None` if `index` is out of bounds.

Element at index 0 is the front of the queue.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
assert_eq!(buf.swap_remove_front(0), None);
buf.push_back(1);
buf.push_back(2);
buf.push_back(3);
assert_eq!(buf, [1, 2, 3]);

assert_eq!(buf.swap_remove_front(2), Some(3));
assert_eq!(buf, [2, 1]);
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2436)

#### pub fn [swap\_remove\_back](#method.swap_remove_back)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Removes an element from anywhere in the deque and returns it, replacing it with the last element.

This does not preserve ordering, but is _O_(1).

Returns `None` if `index` is out of bounds.

Element at index 0 is the front of the queue.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
assert_eq!(buf.swap_remove_back(0), None);
buf.push_back(1);
buf.push_back(2);
buf.push_back(3);
assert_eq!(buf, [1, 2, 3]);

assert_eq!(buf.swap_remove_back(0), Some(1));
assert_eq!(buf, [3, 2]);
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2473)

#### pub fn [insert](#method.insert)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), value: T)

Inserts an element at `index` within the deque, shifting all elements with indices greater than or equal to `index` towards the back.

Element at index 0 is the front of the queue.

##### Panics

Panics if `index` is strictly greater than the deque’s length.

##### Examples

```rust
use std::collections::VecDeque;

let mut vec_deque = VecDeque::new();
vec_deque.push_back('a');
vec_deque.push_back('b');
vec_deque.push_back('c');
assert_eq!(vec_deque, &['a', 'b', 'c']);

vec_deque.insert(1, 'd');
assert_eq!(vec_deque, &['a', 'd', 'b', 'c']);

vec_deque.insert(4, 'e');
assert_eq!(vec_deque, &['a', 'd', 'b', 'c', 'e']);
```

1.95.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2500)

#### pub fn [insert\_mut](#method.insert_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), value: T) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Inserts an element at `index` within the deque, shifting all elements with indices greater than or equal to `index` towards the back, and returning a reference to it.

Element at index 0 is the front of the queue.

##### Panics

Panics if `index` is strictly greater than the deque’s length.

##### Examples

```rust
use std::collections::VecDeque;

let mut vec_deque = VecDeque::from([1, 2, 3]);

let x = vec_deque.insert_mut(1, 5);
*x += 7;
assert_eq!(vec_deque, &[1, 12, 2, 3]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2552)

#### pub fn [remove](#method.remove)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Removes and returns the element at `index` from the deque. Whichever end is closer to the removal point will be moved to make room, and all the affected elements will be moved to new positions. Returns `None` if `index` is out of bounds.

Element at index 0 is the front of the queue.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.push_back('a');
buf.push_back('b');
buf.push_back('c');
assert_eq!(buf, ['a', 'b', 'c']);

assert_eq!(buf.remove(1), Some('b'));
assert_eq!(buf, ['a', 'c']);
```

1.4.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2604-2606)

#### pub fn [split\_off](#method.split_off)(&mut self, at: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T, A>

where A: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Splits the deque into two at the given index.

Returns a newly allocated `VecDeque`. `self` contains elements `[0, at)`, and the returned deque contains elements `[at, len)`.

Note that the capacity of `self` does not change.

Element at index 0 is the front of the queue.

##### Panics

Panics if `at > len`.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf: VecDeque<_> = ['a', 'b', 'c'].into();
let buf2 = buf.split_off(1);
assert_eq!(buf, ['a']);
assert_eq!(buf2, ['b', 'c']);
```

1.4.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2670)

#### pub fn [append](#method.append)(&mut self, other: &mut [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T, A>)

Moves all the elements of `other` into `self`, leaving `other` empty.

##### Panics

Panics if the new number of elements in self overflows a `usize`.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf: VecDeque<_> = [1, 2].into();
let mut buf2: VecDeque<_> = [3, 4].into();
buf.append(&mut buf2);
assert_eq!(buf, [1, 2, 3, 4]);
assert_eq!(buf2, []);
```

1.4.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2725-2727)

#### pub fn [retain](#method.retain)<F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Retains only the elements specified by the predicate.

In other words, remove all elements `e` for which `f(&e)` returns false. This method operates in place, visiting each element exactly once in the original order, and preserves the order of the retained elements.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.extend(1..5);
buf.retain(|&x| x % 2 == 0);
assert_eq!(buf, [2, 4]);
```

Because the elements are visited exactly once in the original order, external state may be used to decide which elements to keep.

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.extend(1..6);

let keep = [false, true, true, false, true];
let mut iter = keep.iter();
buf.retain(|_| *iter.next().unwrap());
assert_eq!(buf, [2, 3, 5]);
```

1.61.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2754-2756)

#### pub fn [retain\_mut](#method.retain_mut)<F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Retains only the elements specified by the predicate.

In other words, remove all elements `e` for which `f(&mut e)` returns false. This method operates in place, visiting each element exactly once in the original order, and preserves the order of the retained elements.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.extend(1..5);
buf.retain_mut(|x| if *x % 2 == 0 {
    *x += 1;
    true
} else {
    false
});
assert_eq!(buf, [3, 5]);
```

1.33.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2830)

#### pub fn [resize\_with](#method.resize_with)(&mut self, new\_len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), generator: impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")() -> T)

Modifies the deque in-place so that `len()` is equal to `new_len`, either by removing excess elements from the back or by appending elements generated by calling `generator` to the back.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.push_back(5);
buf.push_back(10);
buf.push_back(15);
assert_eq!(buf, [5, 10, 15]);

buf.resize_with(5, Default::default);
assert_eq!(buf, [5, 10, 15, 0, 0]);

buf.resize_with(2, || unreachable!());
assert_eq!(buf, [5, 10]);

let mut state = 100;
buf.resize_with(5, || { state += 1; state });
assert_eq!(buf, [5, 10, 101, 102, 103]);
```

1.48.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#2896)

#### pub fn [make\_contiguous](#method.make_contiguous)(&mut self) -> &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

Rearranges the internal storage of this deque so it is one contiguous slice, which is then returned.

This method does not allocate and does not change the order of the inserted elements. As it returns a mutable slice, this can be used to sort a deque.

Once the internal storage is contiguous, the [`as_slices`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.as_slices "method alloc::collections::vec_deque::VecDeque::as_slices") and [`as_mut_slices`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.as_mut_slices "method alloc::collections::vec_deque::VecDeque::as_mut_slices") methods will return the entire contents of the deque in a single slice.

##### Examples

Sorting the content of a deque.

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::with_capacity(15);

buf.push_back(2);
buf.push_back(1);
buf.push_front(3);

// sorting the deque
buf.make_contiguous().sort();
assert_eq!(buf.as_slices(), (&[1, 2, 3] as &[_], &[] as &[_]));

// sorting it in reverse order
buf.make_contiguous().sort_by(|a, b| b.cmp(a));
assert_eq!(buf.as_slices(), (&[3, 2, 1] as &[_], &[] as &[_]));
```

Getting immutable access to the contiguous slice.

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();

buf.push_back(2);
buf.push_back(1);
buf.push_front(3);

buf.make_contiguous();
if let (slice, &[]) = buf.as_slices() {
    // we can now be sure that `slice` contains all elements of the deque,
    // while still having immutable access to `buf`.
    assert_eq!(buf.len(), slice.len());
    assert_eq!(slice, &[3, 2, 1] as &[_]);
}
```

1.36.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#3073)

#### pub fn [rotate\_left](#method.rotate_left)(&mut self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Rotates the double-ended queue `n` places to the left.

Equivalently,

*   Rotates item `n` into the first position.
*   Pops the first `n` items and pushes them to the end.
*   Rotates `len() - n` places to the right.

##### Panics

If `n` is greater than `len()`. Note that `n == len()` does _not_ panic and is a no-op rotation.

##### Complexity

Takes `*O*(min(n, len() - n))` time and no extra space.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf: VecDeque<_> = (0..10).collect();

buf.rotate_left(3);
assert_eq!(buf, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

for i in 1..10 {
    assert_eq!(i * 3 % 10, buf[0]);
    buf.rotate_left(3);
}
assert_eq!(buf, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
```

1.36.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#3116)

#### pub fn [rotate\_right](#method.rotate_right)(&mut self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Rotates the double-ended queue `n` places to the right.

Equivalently,

*   Rotates the first item into position `n`.
*   Pops the last `n` items and pushes them to the front.
*   Rotates `len() - n` places to the left.

##### Panics

If `n` is greater than `len()`. Note that `n == len()` does _not_ panic and is a no-op rotation.

##### Complexity

Takes `*O*(min(n, len() - n))` time and no extra space.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf: VecDeque<_> = (0..10).collect();

buf.rotate_right(3);
assert_eq!(buf, [7, 8, 9, 0, 1, 2, 3, 4, 5, 6]);

for i in 1..10 {
    assert_eq!(0, buf[i * 3 % 10]);
    buf.rotate_right(3);
}
assert_eq!(buf, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
```

1.54.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#3201-3203)

#### pub fn [binary\_search](#method.binary_search)(&self, x: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Binary searches this `VecDeque` for a given element. If the `VecDeque` is not sorted, the returned result is unspecified and meaningless.

If the value is found then [`Result::Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. If the value is not found then [`Result::Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, containing the index where a matching element could be inserted while maintaining sorted order.

See also [`binary_search_by`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.binary_search_by "method alloc::collections::vec_deque::VecDeque::binary_search_by"), [`binary_search_by_key`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.binary_search_by_key "method alloc::collections::vec_deque::VecDeque::binary_search_by_key"), and [`partition_point`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.partition_point "method alloc::collections::vec_deque::VecDeque::partition_point").

##### Examples

Looks up a series of four elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in `[1, 4]`.

```rust
use std::collections::VecDeque;

let deque: VecDeque<_> = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();

assert_eq!(deque.binary_search(&13),  Ok(9));
assert_eq!(deque.binary_search(&4),   Err(7));
assert_eq!(deque.binary_search(&100), Err(13));
let r = deque.binary_search(&1);
assert!(matches!(r, Ok(1..=4)));
```

If you want to insert an item to a sorted deque, while maintaining sort order, consider using [`partition_point`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.partition_point "method alloc::collections::vec_deque::VecDeque::partition_point"):

```rust
use std::collections::VecDeque;

let mut deque: VecDeque<_> = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();
let num = 42;
let idx = deque.partition_point(|&x| x <= num);
// If `num` is unique, `s.partition_point(|&x| x < num)` (with `<`) is equivalent to
// `s.binary_search(&num).unwrap_or_else(|x| x)`, but using `<=` may allow `insert`
// to shift less elements.
deque.insert(idx, num);
assert_eq!(deque, &[0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
```

1.54.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#3247-3249)

#### pub fn [binary\_search\_by](#method.binary_search_by)<'a, F>(&'a self, f: F) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering"),

Binary searches this `VecDeque` with a comparator function.

The comparator function should return an order code that indicates whether its argument is `Less`, `Equal` or `Greater` the desired target. If the `VecDeque` is not sorted or if the comparator function does not implement an order consistent with the sort order of the underlying `VecDeque`, the returned result is unspecified and meaningless.

If the value is found then [`Result::Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. If the value is not found then [`Result::Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, containing the index where a matching element could be inserted while maintaining sorted order.

See also [`binary_search`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.binary_search "method alloc::collections::vec_deque::VecDeque::binary_search"), [`binary_search_by_key`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.binary_search_by_key "method alloc::collections::vec_deque::VecDeque::binary_search_by_key"), and [`partition_point`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.partition_point "method alloc::collections::vec_deque::VecDeque::partition_point").

##### Examples

Looks up a series of four elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in `[1, 4]`.

```rust
use std::collections::VecDeque;

let deque: VecDeque<_> = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();

assert_eq!(deque.binary_search_by(|x| x.cmp(&13)),  Ok(9));
assert_eq!(deque.binary_search_by(|x| x.cmp(&4)),   Err(7));
assert_eq!(deque.binary_search_by(|x| x.cmp(&100)), Err(13));
let r = deque.binary_search_by(|x| x.cmp(&1));
assert!(matches!(r, Ok(1..=4)));
```

1.54.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#3305-3308)

#### pub fn [binary\_search\_by\_key](#method.binary_search_by_key)<'a, B, F>( &'a self, b: [&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html), f: F, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> B, B: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

Binary searches this `VecDeque` with a key extraction function.

Assumes that the deque is sorted by the key, for instance with [`make_contiguous().sort_by_key()`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.make_contiguous "method alloc::collections::vec_deque::VecDeque::make_contiguous") using the same key extraction function. If the deque is not sorted by the key, the returned result is unspecified and meaningless.

If the value is found then [`Result::Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. If the value is not found then [`Result::Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") is returned, containing the index where a matching element could be inserted while maintaining sorted order.

See also [`binary_search`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.binary_search "method alloc::collections::vec_deque::VecDeque::binary_search"), [`binary_search_by`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.binary_search_by "method alloc::collections::vec_deque::VecDeque::binary_search_by"), and [`partition_point`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.partition_point "method alloc::collections::vec_deque::VecDeque::partition_point").

##### Examples

Looks up a series of four elements in a slice of pairs sorted by their second elements. The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in `[1, 4]`.

```rust
use std::collections::VecDeque;

let deque: VecDeque<_> = [(0, 0), (2, 1), (4, 1), (5, 1),
         (3, 1), (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
         (1, 21), (2, 34), (4, 55)].into();

assert_eq!(deque.binary_search_by_key(&13, |&(a, b)| b),  Ok(9));
assert_eq!(deque.binary_search_by_key(&4, |&(a, b)| b),   Err(7));
assert_eq!(deque.binary_search_by_key(&100, |&(a, b)| b), Err(13));
let r = deque.binary_search_by_key(&1, |&(a, b)| b);
assert!(matches!(r, Ok(1..=4)));
```

1.54.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#3357-3359)

#### pub fn [partition\_point](#method.partition_point)<P>(&self, pred: P) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

where P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Returns the index of the partition point according to the given predicate (the index of the first element of the second partition).

The deque is assumed to be partitioned according to the given predicate. This means that all elements for which the predicate returns true are at the start of the deque and all elements for which the predicate returns false are at the end. For example, `[7, 15, 3, 5, 4, 12, 6]` is partitioned under the predicate `x % 2 != 0` (all odd numbers are at the start, all even at the end).

If the deque is not partitioned, the returned result is unspecified and meaningless, as this method performs a kind of binary search.

See also [`binary_search`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.binary_search "method alloc::collections::vec_deque::VecDeque::binary_search"), [`binary_search_by`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.binary_search_by "method alloc::collections::vec_deque::VecDeque::binary_search_by"), and [`binary_search_by_key`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html#method.binary_search_by_key "method alloc::collections::vec_deque::VecDeque::binary_search_by_key").

##### Examples

```rust
use std::collections::VecDeque;

let deque: VecDeque<_> = [1, 2, 3, 3, 5, 6, 7].into();
let i = deque.partition_point(|&x| x < 5);

assert_eq!(i, 4);
assert!(deque.iter().take(i).all(|&x| x < 5));
assert!(deque.iter().skip(i).all(|&x| !(x < 5)));
```

If you want to insert an item to a sorted deque, while maintaining sort order:

```rust
use std::collections::VecDeque;

let mut deque: VecDeque<_> = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();
let num = 42;
let idx = deque.partition_point(|&x| x < num);
deque.insert(idx, num);
assert_eq!(deque, &[0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
```

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#3394)

#### pub fn [resize](#method.resize)(&mut self, new\_len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), value: T)

Modifies the deque in-place so that `len()` is equal to new\_len, either by removing excess elements from the back or by appending clones of `value` to the back.

##### Examples

```rust
use std::collections::VecDeque;

let mut buf = VecDeque::new();
buf.push_back(5);
buf.push_back(10);
buf.push_back(15);
assert_eq!(buf, [5, 10, 15]);

buf.resize(2, 0);
assert_eq!(buf, [5, 10]);

buf.resize(5, 20);
assert_eq!(buf, [5, 10, 20, 20, 20]);
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#3430-3432)

#### pub fn [extend\_from\_within](#method.extend_from_within)<R>(&mut self, src: R)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

🔬This is a nightly-only experimental API. (`deque_extend_front`)

Available on **non-`no_global_oom_handling`** only.

Clones the elements at the range `src` and appends them to the end.

##### Panics

Panics if the starting index is greater than the end index or if either index is greater than the length of the vector.

##### Examples

```rust
#![feature(deque_extend_front)]
use std::collections::VecDeque;

let mut characters = VecDeque::from(['a', 'b', 'c', 'd', 'e']);
characters.extend_from_within(2..);
assert_eq!(characters, ['a', 'b', 'c', 'd', 'e', 'c', 'd', 'e']);

let mut numbers = VecDeque::from([0, 1, 2, 3, 4]);
numbers.extend_from_within(..2);
assert_eq!(numbers, [0, 1, 2, 3, 4, 0, 1]);

let mut strings = VecDeque::from([String::from("hello"), String::from("world"), String::from("!")]);
strings.extend_from_within(1..=2);
assert_eq!(strings, ["hello", "world", "!", "world", "!"]);
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/collections/vec_deque/mod.rs.html#3472-3474)

#### pub fn [prepend\_from\_within](#method.prepend_from_within)<R>(&mut self, src: R)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

🔬This is a nightly-only experimental API. (`deque_extend_front`)

Available on **non-`no_global_oom_handling`** only.

Clones the elements at the range `src` and prepends them to the front.

##### Panics

Panics if the starting index is greater than the end index or if either index is greater than the length of the vector.

##### Examples

```rust
#![feature(deque_extend_front)]
use std::collections::VecDeque;

let mut characters = VecDeque::from(['a', 'b', 'c', 'd', 'e']);
characters.prepend_from_within(2..);
assert_eq!(characters, ['c', 'd', 'e', 'a', 'b', 'c', 'd', 'e']);

let mut numbers = VecDeque::from([0, 1, 2, 3, 4]);
numbers.prepend_from_within(..2);
assert_eq!(numbers, [0, 1, 0, 1, 2, 3, 4]);

let mut strings = VecDeque::from([String::from("hello"), String::from("world"), String::from("!")]);
strings.prepend_from_within(1..=2);
assert_eq!(strings, ["world", "!", "hello", "world", "!"]);
```

## Trait Implementations

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#45)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#45)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#45)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#45)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#45)

### impl [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#45)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<[ActionRequest](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.ActionRequest.html "struct accesskit::ActionRequest")\>

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#45)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#45)

### impl [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#45)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

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

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#404)

### impl<T> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

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

### impl<T> [Instrument](../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.in_current_span)

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

### impl<T> [IntoResult](../../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#347)

### impl<R, P> [ReadPrimitive](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html "trait lebe::io::ReadPrimitive")<R> for P

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<P>, P: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#377)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

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

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#method.clone_into)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

Creates a type-erased clone of this value.

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

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Drain<'\_, T, A>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/drain/struct.Drain.html\\" title=\\"struct alloc::collections::vec\_deque::drain::Drain\\">Drain</a>&lt;'\_, T, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/drain/struct.Drain.html\\" title=\\"struct alloc::collections::vec\_deque::drain::Drain\\">Drain</a>&lt;'\_, T, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","ExtractIf<'\_, T, F, A>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/extract\_if/struct.ExtractIf.html\\" title=\\"struct alloc::collections::vec\_deque::extract\_if::ExtractIf\\">ExtractIf</a>&lt;'\_, T, F, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, F, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/extract\_if/struct.ExtractIf.html\\" title=\\"struct alloc::collections::vec\_deque::extract\_if::ExtractIf\\">ExtractIf</a>&lt;'\_, T, F, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;mut T</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Iter<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/iter/struct.Iter.html\\" title=\\"struct alloc::collections::vec\_deque::iter::Iter\\">Iter</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/iter/struct.Iter.html\\" title=\\"struct alloc::collections::vec\_deque::iter::Iter\\">Iter</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>;</div>","IterMut<'\_, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/iter\_mut/struct.IterMut.html\\" title=\\"struct alloc::collections::vec\_deque::iter\_mut::IterMut\\">IterMut</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/iter\_mut/struct.IterMut.html\\" title=\\"struct alloc::collections::vec\_deque::iter\_mut::IterMut\\">IterMut</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a mut T</a>;</div>","Splice<'\_, <I as IntoIterator>::IntoIter, A>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/splice/struct.Splice.html\\" title=\\"struct alloc::collections::vec\_deque::splice::Splice\\">Splice</a>&lt;'\_, I, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;I, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/splice/struct.Splice.html\\" title=\\"struct alloc::collections::vec\_deque::splice::Splice\\">Splice</a>&lt;'\_, I, A&gt;<div class=\\"where\\">where\\n I: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;I as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}