[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[rts\_array](index.html)

# Trait Truncate 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#67)

```rust
pub trait Truncate {
    // Required method
    fn truncate(&mut self, _len: usize);
}
```

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#68)

#### fn [truncate](#tymethod.truncate)(&mut self, \_len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#255)

### impl<T> [Truncate](trait.Truncate.html "trait bevy::render::render_resource::encase::rts_array::Truncate") for [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#256)

#### fn [truncate](#tymethod.truncate)(&mut self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

### impl<T> [Truncate](trait.Truncate.html "trait bevy::render::render_resource::encase::rts_array::Truncate") for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

#### fn [truncate](#tymethod.truncate)(&mut self, len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

## Implementors

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#251)

### impl<T> [Truncate](trait.Truncate.html "trait bevy::render::render_resource::encase::rts_array::Truncate") for [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>