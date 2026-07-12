[bevy](../../index.html)::[ecs](../index.html)::[ptr](index.html)

# Trait UnsafeCellDeref 

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#1198)

```rust
pub trait UnsafeCellDeref<'a, T>: SealedUnsafeCell {
    // Required methods
    unsafe fn deref_mut(self) -> &'a mut T;
    unsafe fn deref(self) -> &'a T;
    unsafe fn read(self) -> T
       where T: Copy;
}
```

Extension trait for helper methods on [`UnsafeCell`](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html "struct core::cell::UnsafeCell")

## Required Methods

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#1202)

#### unsafe fn [deref\_mut](#tymethod.deref_mut)(self) -> [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

##### Safety

*   The returned value must be unique and not alias any mutable or immutable references to the contents of the [`UnsafeCell`](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html "struct core::cell::UnsafeCell").
*   At all times, you must avoid data races. If multiple threads have access to the same [`UnsafeCell`](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html "struct core::cell::UnsafeCell"), then any writes must have a proper happens-before relation to all other accesses or use atomics ([`UnsafeCell`](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html "struct core::cell::UnsafeCell") docs for reference).

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#1207)

#### unsafe fn [deref](#tymethod.deref)(self) -> [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

##### Safety

*   For the lifetime `'a` of the returned value you must not construct a mutable reference to the contents of the [`UnsafeCell`](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html "struct core::cell::UnsafeCell").
*   At all times, you must avoid data races. If multiple threads have access to the same [`UnsafeCell`](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html "struct core::cell::UnsafeCell"), then any writes must have a proper happens-before relation to all other accesses or use atomics ([`UnsafeCell`](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html "struct core::cell::UnsafeCell") docs for reference).

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#1214-1216)

#### unsafe fn [read](#tymethod.read)(self) -> T

where T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"),

Returns a copy of the contained value.

##### Safety

*   The [`UnsafeCell`](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html "struct core::cell::UnsafeCell") must not currently have a mutable reference to its content.
*   At all times, you must avoid data races. If multiple threads have access to the same [`UnsafeCell`](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html "struct core::cell::UnsafeCell"), then any writes must have a proper happens-before relation to all other accesses or use atomics ([`UnsafeCell`](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html "struct core::cell::UnsafeCell") docs for reference).

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#1219)

### impl<'a, T> [UnsafeCellDeref](trait.UnsafeCellDeref.html "trait bevy::ecs::ptr::UnsafeCellDeref")<'a, T> for &'a [UnsafeCell](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html "struct core::cell::UnsafeCell")<T>

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#1221)

#### unsafe fn [deref\_mut](#tymethod.deref_mut)(self) -> [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#1226)

#### unsafe fn [deref](#tymethod.deref)(self) -> [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#1232-1234)

#### unsafe fn [read](#tymethod.read)(self) -> T

where T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"),

## Implementors