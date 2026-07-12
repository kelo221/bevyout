[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[rts\_array](index.html)

# Trait Length 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#63)

```rust
pub trait Length {
    // Required method
    fn length(&self) -> usize;
}
```

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#64)

#### fn [length](#tymethod.length)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#253)

### impl<T> [Length](trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length") for [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#253)

#### fn [length](#tymethod.length)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

### impl<T> [Length](trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length") for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

#### fn [length](#tymethod.length)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#250)

### impl<T> [Length](trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length") for [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#250)

#### fn [length](#tymethod.length)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

## Implementors

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#251)

### impl<T> [Length](trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length") for [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>