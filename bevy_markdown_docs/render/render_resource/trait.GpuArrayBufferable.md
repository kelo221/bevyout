[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait GpuArrayBufferable 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/gpu_array_buffer.rs.html#18)

```rust
pub trait GpuArrayBufferable:
    ShaderType
    + ShaderSize
    + WriteInto
    + Clone { }
```

Trait for types able to go in a [`GpuArrayBuffer`](enum.GpuArrayBuffer.html "enum bevy::render::render_resource::GpuArrayBuffer").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/gpu_array_buffer.rs.html#20)

### impl<T> [GpuArrayBufferable](trait.GpuArrayBufferable.html "trait bevy::render::render_resource::GpuArrayBufferable") for T

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize") + [WriteInto](encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),