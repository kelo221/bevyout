[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias TextureDescriptor 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/texture.rs.html#187)

```rust
pub type TextureDescriptor<'a> = TextureDescriptor<Option<&'a str>, &'a [TextureFormat]>;
```

Describes a [`Texture`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/texture/struct.Texture.html "struct wgpu::api::texture::Texture").

For use with [`Device::create_texture`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html#method.create_texture "method wgpu::api::device::Device::create_texture").

Corresponds to [WebGPU `GPUTextureDescriptor`](https://gpuweb.github.io/gpuweb/#dictdef-gputexturedescriptor).

## Aliased Type

```rust
#[repr(C)]pub struct TextureDescriptor<'a> {
    pub label: Option<&'a str>,
    pub size: Extent3d,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsages,
    pub view_formats: &'a [TextureFormat],
}
```

## Fields

`label: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>`

Debug label of the texture. This will show up in graphics debuggers for easy identification.

`size: [Extent3d](struct.Extent3d.html "struct bevy::render::render_resource::Extent3d")`

Size of the texture. All components must be greater than zero. For a regular 1D/2D texture, the unused sizes will be 1. For 2DArray textures, Z is the number of 2D textures in that array.

`mip_level_count: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)`

Mip count of texture. For a texture with no extra mips, this must be 1.

`sample_count: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)`

Sample count of texture. If this is not 1, texture must have \[`BindingType::Texture::multisampled`\] set to true.

`dimension: [TextureDimension](enum.TextureDimension.html "enum bevy::render::render_resource::TextureDimension")`

Dimensions of the texture.

`format: [TextureFormat](enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")`

Format of the texture.

`usage: [TextureUsages](struct.TextureUsages.html "struct bevy::render::render_resource::TextureUsages")`

Allowed usages of the texture. If used in other ways, the operation will panic.

`view_formats: &'a [[TextureFormat](enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")]`

Specifies what view formats will be allowed when calling `Texture::create_view` on this texture.

View formats of the same format as the texture are always allowed.

Note: currently, only the srgb-ness is allowed to change. (ex: `Rgba8Unorm` texture + `Rgba8UnormSrgb` view)