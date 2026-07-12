[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait AsBindGroupShaderType 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#690)

```rust
pub trait AsBindGroupShaderType<T>where
    T: ShaderType,{
    // Required method
    fn as_bind_group_shader_type(&self, images: &RenderAssets<GpuImage>) -> T;
}
```

Converts a value to a [`ShaderType`](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for use in a bind group.

This is automatically implemented for references that implement [`Into`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into"). Generally normal [`Into`](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into") / [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") impls should be preferred, but sometimes additional runtime metadata is required. This exists largely to make some [`AsBindGroup`](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") use cases easier.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#693)

#### fn [as\_bind\_group\_shader\_type](#tymethod.as_bind_group_shader_type)(&self, images: &[RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> T

Return the `T` [`ShaderType`](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#121)

### impl [AsBindGroupShaderType](trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<[ColorMaterialUniform](../../sprite_render/struct.ColorMaterialUniform.html "struct bevy::sprite_render::ColorMaterialUniform")\> for [ColorMaterial](../../prelude/struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#102)

### impl [AsBindGroupShaderType](trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<[ForwardDecalMaterialExtUniform](../../pbr/decal/struct.ForwardDecalMaterialExtUniform.html "struct bevy::pbr::decal::ForwardDecalMaterialExtUniform")\> for [ForwardDecalMaterialExt](../../pbr/decal/struct.ForwardDecalMaterialExt.html "struct bevy::pbr::decal::ForwardDecalMaterialExt")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#99)

### impl [AsBindGroupShaderType](trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<[SpriteMaterialUniform](../../sprite_render/struct.SpriteMaterialUniform.html "struct bevy::sprite_render::SpriteMaterialUniform")\> for [SpriteMaterial](../../prelude/struct.SpriteMaterial.html "struct bevy::prelude::SpriteMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#1066)

### impl [AsBindGroupShaderType](trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<[StandardMaterialUniform](../../pbr/struct.StandardMaterialUniform.html "struct bevy::pbr::StandardMaterialUniform")\> for [StandardMaterial](../../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,