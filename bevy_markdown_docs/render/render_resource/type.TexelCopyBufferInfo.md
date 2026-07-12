[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias TexelCopyBufferInfo 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#45)

```rust
pub type TexelCopyBufferInfo<'a> = TexelCopyBufferInfo<&'a Buffer>;
```

View of a buffer which can be used to copy to/from a texture.

Corresponds to [WebGPU `GPUTexelCopyBufferInfo`](https://gpuweb.github.io/gpuweb/#dictdef-gpuimagecopybuffer).

## Aliased Type

```rust
#[repr(C)]pub struct TexelCopyBufferInfo<'a> {
    pub buffer: &'a Buffer,
    pub layout: TexelCopyBufferLayout,
}
```

## Fields

`buffer: &'a [Buffer](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer")`

The buffer to be copied to/from.

`layout: [TexelCopyBufferLayout](struct.TexelCopyBufferLayout.html "struct bevy::render::render_resource::TexelCopyBufferLayout")`

The layout of the texture data in this buffer.