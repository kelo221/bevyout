[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait IntoBinding 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#139)

```rust
pub trait IntoBinding<'a> {
    // Required method
    fn into_binding(self) -> BindingResource<'a>;
}
```

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#140)

#### fn [into\_binding](#tymethod.into_binding)(self) -> [BindingResource](enum.BindingResource.html "enum bevy::render::render_resource::BindingResource")<'a>

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#171)

### impl<'a> [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'a> for &'a \[&'a [Sampler](struct.WgpuSampler.html "struct bevy::render::render_resource::WgpuSampler")\]

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#173)

#### fn [into\_binding](#tymethod.into_binding)(self) -> [BindingResource](enum.BindingResource.html "enum bevy::render::render_resource::BindingResource")<'a>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#157)

### impl<'a> [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'a> for &'a \[&'a [TextureView](struct.WgpuTextureView.html "struct bevy::render::render_resource::WgpuTextureView")\]

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#159)

#### fn [into\_binding](#tymethod.into_binding)(self) -> [BindingResource](enum.BindingResource.html "enum bevy::render::render_resource::BindingResource")<'a>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#192)

### impl<'a> [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'a> for &'a \[[BufferBinding](struct.BufferBinding.html "struct bevy::render::render_resource::BufferBinding")<'a>\]

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#194)

#### fn [into\_binding](#tymethod.into_binding)(self) -> [BindingResource](enum.BindingResource.html "enum bevy::render::render_resource::BindingResource")<'a>

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/storage_buffer.rs.html#283)

### impl<'a, T> [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'a> for &'a [DynamicStorageBuffer](struct.DynamicStorageBuffer.html "struct bevy::render::render_resource::DynamicStorageBuffer")<T>

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [WriteInto](encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/uniform_buffer.rs.html#400)

### impl<'a, T> [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'a> for &'a [DynamicUniformBuffer](struct.DynamicUniformBuffer.html "struct bevy::render::render_resource::DynamicUniformBuffer")<T>

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [WriteInto](encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/storage_buffer.rs.html#152)

### impl<'a, T> [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'a> for &'a [StorageBuffer](struct.StorageBuffer.html "struct bevy::render::render_resource::StorageBuffer")<T>

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [WriteInto](encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/uniform_buffer.rs.html#145)

### impl<'a, T> [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'a> for &'a [UniformBuffer](struct.UniformBuffer.html "struct bevy::render::render_resource::UniformBuffer")<T>

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [WriteInto](encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#164)

### impl<'a> [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'a> for &'a [Sampler](struct.Sampler.html "struct bevy::render::render_resource::Sampler")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#143)

### impl<'a> [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'a> for &'a bevy::render::render\_resource::[TextureView](struct.TextureView.html "struct bevy::render::render_resource::TextureView")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#150)

### impl<'a> [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'a> for &'a bevy::render::render\_resource::[WgpuTextureView](struct.WgpuTextureView.html "struct bevy::render::render_resource::WgpuTextureView")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#178)

### impl<'a> [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'a> for [BindingResource](enum.BindingResource.html "enum bevy::render::render_resource::BindingResource")<'a>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#185)

### impl<'a> [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'a> for [BufferBinding](struct.BufferBinding.html "struct bevy::render::render_resource::BufferBinding")<'a>