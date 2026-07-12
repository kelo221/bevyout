[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[vector](index.html)

# Trait FromVectorParts 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/vector.rs.html#17)

```rust
pub trait FromVectorParts<T, const N: usize>where
    T: VectorScalar,{
    // Required method
    fn from_parts(parts: [T; N]) -> Self;
}
```

Enables the creation of a vector (via `[T; N]`)

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/vector.rs.html#18)

#### fn [from\_parts](#tymethod.from_parts)(parts: [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)) -> Self

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [FromVectorParts](trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2> for [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [FromVectorParts](trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3> for [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [FromVectorParts](trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4> for [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#9)

### impl [FromVectorParts](trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 2> for [IVec2](../../../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

where [IVec2](../../../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#13)

### impl [FromVectorParts](trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 3> for [IVec3](../../../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

where [IVec3](../../../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [FromVectorParts](trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 4> for [IVec4](../../../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [IVec4](../../../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [FromVectorParts](trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 2> for [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#12)

### impl [FromVectorParts](trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 3> for [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

where [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [FromVectorParts](trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 4> for [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

where [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),