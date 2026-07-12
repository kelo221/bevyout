[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[vector](index.html)

# Trait AsRefVectorParts 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/vector.rs.html#7)

```rust
pub trait AsRefVectorParts<T, const N: usize>where
    T: VectorScalar,{
    // Required method
    fn as_ref_parts(&self) -> &[T; N];
}
```

Enables reading from the vector (via `&[T; N]`)

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/vector.rs.html#8)

#### fn [as\_ref\_parts](#tymethod.as_ref_parts)(&self) -> &[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [AsRefVectorParts](trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2> for [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [AsRefVectorParts](trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3> for [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [AsRefVectorParts](trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4> for [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#9)

### impl [AsRefVectorParts](trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 2> for [IVec2](../../../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

where [IVec2](../../../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#13)

### impl [AsRefVectorParts](trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 3> for [IVec3](../../../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

where [IVec3](../../../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [AsRefVectorParts](trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 4> for [IVec4](../../../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [IVec4](../../../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [AsRefVectorParts](trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 2> for [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#12)

### impl [AsRefVectorParts](trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 3> for [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

where [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [AsRefVectorParts](trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 4> for [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

where [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),