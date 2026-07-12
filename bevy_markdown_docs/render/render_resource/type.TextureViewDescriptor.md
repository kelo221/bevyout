[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias TextureViewDescriptor 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/texture_view.rs.html#91)

```rust
pub type TextureViewDescriptor<'a> = TextureViewDescriptor<Option<&'a str>>;
```

Describes a [`TextureView`](struct.WgpuTextureView.html "struct bevy::render::render_resource::WgpuTextureView").

For use with [`Texture::create_view`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/texture/struct.Texture.html#method.create_view "method wgpu::api::texture::Texture::create_view").

Corresponds to [WebGPU `GPUTextureViewDescriptor`](https://gpuweb.github.io/gpuweb/#dictdef-gputextureviewdescriptor).

## Aliased Type

```rust
pub struct TextureViewDescriptor<'a> {
    pub label: Option<&'a str>,
    pub format: Option<TextureFormat>,
    pub dimension: Option<TextureViewDimension>,
    pub usage: Option<TextureUsages>,
    pub aspect: TextureAspect,
    pub base_mip_level: u32,
    pub mip_level_count: Option<u32>,
    pub base_array_layer: u32,
    pub array_layer_count: Option<u32>,
}
```

## Fields

`label: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>`

Debug label of the texture view. This will show up in graphics debuggers for easy identification.

`format: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TextureFormat](enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")>`

Format of the texture view. Either must be the same as the texture format or in the list of `view_formats` in the texture’s descriptor.

`dimension: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TextureViewDimension](enum.TextureViewDimension.html "enum bevy::render::render_resource::TextureViewDimension")>`

The dimension of the texture view. For 1D textures, this must be `D1`. For 2D textures it must be one of `D2`, `D2Array`, `Cube`, and `CubeArray`. For 3D textures it must be `D3`

`usage: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TextureUsages](struct.TextureUsages.html "struct bevy::render::render_resource::TextureUsages")>`

The allowed usage(s) for the texture view. Must be a subset of the usage flags of the texture. If not provided, defaults to the full set of usage flags of the texture.

`aspect: [TextureAspect](enum.TextureAspect.html "enum bevy::render::render_resource::TextureAspect")`

Aspect of the texture. Color textures must be \[`TextureAspect::All`\].

`base_mip_level: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)`

Base mip level.

`mip_level_count: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)>`

Mip level count. If `Some(count)`, `base_mip_level + count` must be less or equal to underlying texture mip count. If `None`, considered to include the rest of the mipmap levels, but at least 1 in total.

`base_array_layer: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)`

Base array layer.

`array_layer_count: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)>`

Layer count. If `Some(count)`, `base_array_layer + count` must be less or equal to the underlying array count. If `None`, considered to include the rest of the array layers, but at least 1 in total.