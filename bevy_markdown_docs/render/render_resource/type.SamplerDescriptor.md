[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias SamplerDescriptor 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/sampler.rs.html#35)

```rust
pub type SamplerDescriptor<'a> = SamplerDescriptor<Option<&'a str>>;
```

Describes a [`Sampler`](struct.WgpuSampler.html "struct bevy::render::render_resource::WgpuSampler").

For use with [`Device::create_sampler`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html#method.create_sampler "method wgpu::api::device::Device::create_sampler").

Corresponds to [WebGPU `GPUSamplerDescriptor`](https://gpuweb.github.io/gpuweb/#dictdef-gpusamplerdescriptor).

## Aliased Type

```rust
pub struct SamplerDescriptor<'a> {
    pub label: Option<&'a str>,
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: MipmapFilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<CompareFunction>,
    pub anisotropy_clamp: u16,
    pub border_color: Option<SamplerBorderColor>,
}
```

## Fields

`label: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>`

Debug label of the sampler. This will show up in graphics debuggers for easy identification.

`address_mode_u: [AddressMode](enum.AddressMode.html "enum bevy::render::render_resource::AddressMode")`

How to deal with out of bounds accesses in the u (i.e. x) direction

`address_mode_v: [AddressMode](enum.AddressMode.html "enum bevy::render::render_resource::AddressMode")`

How to deal with out of bounds accesses in the v (i.e. y) direction

`address_mode_w: [AddressMode](enum.AddressMode.html "enum bevy::render::render_resource::AddressMode")`

How to deal with out of bounds accesses in the w (i.e. z) direction

`mag_filter: [FilterMode](enum.FilterMode.html "enum bevy::render::render_resource::FilterMode")`

How to filter the texture when it needs to be magnified (made larger)

`min_filter: [FilterMode](enum.FilterMode.html "enum bevy::render::render_resource::FilterMode")`

How to filter the texture when it needs to be minified (made smaller)

`mipmap_filter: [MipmapFilterMode](enum.MipmapFilterMode.html "enum bevy::render::render_resource::MipmapFilterMode")`

How to filter between mip map levels

`lod_min_clamp: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)`

Minimum level of detail (i.e. mip level) to use

`lod_max_clamp: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)`

Maximum level of detail (i.e. mip level) to use

`compare: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[CompareFunction](enum.CompareFunction.html "enum bevy::render::render_resource::CompareFunction")>`

If this is enabled, this is a comparison sampler using the given comparison function.

`anisotropy_clamp: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)`

Must be at least 1. If this is not 1, all filter modes must be linear.

`border_color: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[SamplerBorderColor](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/enum.SamplerBorderColor.html "enum wgpu_types::texture::SamplerBorderColor")>`

Border color to use when `address_mode` is \[`AddressMode::ClampToBorder`\]