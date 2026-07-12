[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias CommandEncoderDescriptor 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/command_encoder.rs.html#37)

```rust
pub type CommandEncoderDescriptor<'a> = CommandEncoderDescriptor<Option<&'a str>>;
```

Describes a [`CommandEncoder`](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder").

For use with [`Device::create_command_encoder`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html#method.create_command_encoder "method wgpu::api::device::Device::create_command_encoder").

Corresponds to [WebGPU `GPUCommandEncoderDescriptor`](https://gpuweb.github.io/gpuweb/#dictdef-gpucommandencoderdescriptor).

## Aliased Type

```rust
#[repr(C)]pub struct CommandEncoderDescriptor<'a> {
    pub label: Option<&'a str>,
}
```

## Fields

`label: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>`

Debug label for the command encoder. This will show up in graphics debuggers for easy identification.