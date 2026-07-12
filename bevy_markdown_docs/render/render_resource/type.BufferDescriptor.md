[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias BufferDescriptor 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/buffer.rs.html#846)

```rust
pub type BufferDescriptor<'a> = BufferDescriptor<Option<&'a str>>;
```

Describes a [`Buffer`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer").

For use with [`Device::create_buffer`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html#method.create_buffer "method wgpu::api::device::Device::create_buffer").

Corresponds to [WebGPU `GPUBufferDescriptor`](https://gpuweb.github.io/gpuweb/#dictdef-gpubufferdescriptor).

## Aliased Type

```rust
#[repr(C)]pub struct BufferDescriptor<'a> {
    pub label: Option<&'a str>,
    pub size: u64,
    pub usage: BufferUsages,
    pub mapped_at_creation: bool,
}
```

## Fields

`label: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>`

Debug label of a buffer. This will show up in graphics debuggers for easy identification.

`size: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)`

Size of a buffer, in bytes.

`usage: [BufferUsages](struct.BufferUsages.html "struct bevy::render::render_resource::BufferUsages")`

Usages of a buffer. If the buffer is used in any way that isn’t specified here, the operation will panic.

Specifying only usages the application will actually perform may increase performance. Additionally, on the WebGL backend, there are restrictions on \[`BufferUsages::INDEX`\]; see \[`DownlevelFlags::UNRESTRICTED_INDEX_BUFFER`\] for more information.

`mapped_at_creation: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)`

Allows a buffer to be mapped immediately after they are made. It does not have to be \[`BufferUsages::MAP_READ`\] or \[`BufferUsages::MAP_WRITE`\], all buffers are allowed to be mapped at creation.

If this is `true`, [`size`](#structfield.size) must be a multiple of \[`COPY_BUFFER_ALIGNMENT`\].