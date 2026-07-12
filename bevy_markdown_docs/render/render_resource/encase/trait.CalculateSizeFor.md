[bevy](../../../index.html)::[render](../../index.html)::[render\_resource](../index.html)::[encase](index.html)

# Trait CalculateSizeFor 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#239)

```rust
pub trait CalculateSizeFor {
    // Required method
    fn calculate_size_for(nr_of_el: u64) -> NonZero<u64>;
}
```

Trait implemented for [WGSL runtime-sized arrays](https://gpuweb.github.io/gpuweb/wgsl/#runtime-sized) and [WGSL structs containing runtime-sized arrays](https://gpuweb.github.io/gpuweb/wgsl/#struct-types) (non fixed-footprint types)

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#241)

#### fn [calculate\_size\_for](#tymethod.calculate_size_for)(nr\_of\_el: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns the size of `Self` assuming the (contained) runtime-sized array has `nr_of_el` elements

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#114)

### impl<T> [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#114)

#### fn [calculate\_size\_for](#tymethod.calculate_size_for)(nr\_of\_el: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#115)

### impl<T> [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#115)

#### fn [calculate\_size\_for](#tymethod.calculate_size_for)(nr\_of\_el: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#120)

### impl<T> [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") for [Cell](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html "struct core::cell::Cell")<T>

where T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#120)

#### fn [calculate\_size\_for](#tymethod.calculate_size_for)(nr\_of\_el: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#117)

### impl<T> [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, T>

where T: [ToOwned](../../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned")<Owned = T> + [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#117)

#### fn [calculate\_size\_for](#tymethod.calculate_size_for)(nr\_of\_el: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#253)

### impl<T> [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") for [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>

where [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>: [ShaderType](../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>,

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#253)

#### fn [calculate\_size\_for](#tymethod.calculate_size_for)(nr\_of\_el: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#118)

### impl<T> [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") for [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>

where T: [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#118)

#### fn [calculate\_size\_for](#tymethod.calculate_size_for)(nr\_of\_el: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

### impl<T> [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>: [ShaderType](../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>,

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

#### fn [calculate\_size\_for](#tymethod.calculate_size_for)(nr\_of\_el: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#250)

### impl<T> [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") for [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

where [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html): [ShaderType](../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>,

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#250)

#### fn [calculate\_size\_for](#tymethod.calculate_size_for)(nr\_of\_el: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

## Implementors

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#119)

### impl<T> [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") for [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>

where T: [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#116)

### impl<T> [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#251)

### impl<T> [CalculateSizeFor](trait.CalculateSizeFor.html "trait bevy::render::render_resource::encase::CalculateSizeFor") for [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: [ShaderType](../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>,