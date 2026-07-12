[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias TexelCopyTextureInfo 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#54)

```rust
pub type TexelCopyTextureInfo<'a> = TexelCopyTextureInfo<&'a Texture>;
```

View of a texture which can be used to copy to/from a buffer/texture.

Corresponds to [WebGPU `GPUTexelCopyTextureInfo`](https://gpuweb.github.io/gpuweb/#dictdef-gpuimagecopytexture).

## Aliased Type

```rust
#[repr(C)]pub struct TexelCopyTextureInfo<'a> {
    pub texture: &'a Texture,
    pub mip_level: u32,
    pub origin: Origin3d,
    pub aspect: TextureAspect,
}
```

## Fields

`texture: &'a [Texture](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/texture/struct.Texture.html "struct wgpu::api::texture::Texture")`

The texture to be copied to/from.

`mip_level: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)`

The target mip level of the texture.

`origin: [Origin3d](struct.Origin3d.html "struct bevy::render::render_resource::Origin3d")`

The base texel of the texture in the selected `mip_level`. Together with the `copy_size` argument to copy functions, defines the sub-region of the texture to copy.

`aspect: [TextureAspect](enum.TextureAspect.html "enum bevy::render::render_resource::TextureAspect")`

The copy aspect.